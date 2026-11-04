use serde::Deserialize;

use crate::NodeId;
use crate::geo::{Coordinate, Degrees, Distance, RouteFix, RouteLine};
use crate::itinerary::Problem;
use crate::way::link::Link;
use crate::way::{MappedSegment, RoadNetwork};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MappedRun {
    nodes: Vec<NodeId>,
    from: Option<[Degrees; Coordinate::LON_LAT_LENGTH]>,
    to: Option<[Degrees; Coordinate::LON_LAT_LENGTH]>,
}

impl MappedRun {
    const END_SLACK: Distance = Distance::new(1.5);

    pub fn segment(&self, network: &RoadNetwork) -> Result<MappedSegment, Problem> {
        let places = self
            .nodes
            .iter()
            .map(|node| {
                network
                    .place(*node)
                    .ok_or(Problem::NodeOnNoWay { node: *node })
            })
            .collect::<Result<Vec<Coordinate>, Problem>>()?;
        let [first, second] = places
            .first_chunk::<{ RouteLine::SEGMENT_ENDS_LENGTH }>()
            .ok_or(Problem::ShortMappedSegment)?;
        let [last_but_one, last] = places
            .last_chunk::<{ RouteLine::SEGMENT_ENDS_LENGTH }>()
            .ok_or(Problem::ShortMappedSegment)?;
        let links = self.joined(network)?;
        let middle = places.get(1..places.len() - 1).unwrap_or_default();
        let start = Self::end_on_edge(self.from, *first, *second)?;
        let finish = Self::end_on_edge(self.to, *last, *last_but_one)?;
        Ok(MappedSegment::along(
            std::iter::once(start)
                .chain(middle.iter().copied())
                .chain(std::iter::once(finish))
                .collect(),
            self.nodes.clone(),
            links,
        ))
    }

    fn joined(&self, network: &RoadNetwork) -> Result<Vec<Link>, Problem> {
        self.nodes
            .windows(RouteLine::SEGMENT_ENDS_LENGTH)
            .map(|pair| network.link(pair[0], pair[1]))
            .collect()
    }

    fn end_on_edge(
        written: Option<[Degrees; Coordinate::LON_LAT_LENGTH]>,
        node: Coordinate,
        neighbour: Coordinate,
    ) -> Result<Coordinate, Problem> {
        written
            .map(|[lon, lat]| Coordinate::new(lat, lon))
            .map_or(Ok(node), |point| {
                let fix = RouteFix::on_segment(node, neighbour, Distance::default(), point);
                let off = Distance::from(fix.off_route());
                (off <= Self::END_SLACK)
                    .then_some(point)
                    .ok_or(Problem::EndOffWay { off })
            })
    }
}
