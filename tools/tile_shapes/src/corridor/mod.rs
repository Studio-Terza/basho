use guide::Trip;
use guide::itinerary::Day;
use guide::leg::Mode;
use std::collections::BTreeSet;
use std::fmt;

use map_class::Zoom;
use tiles::GridAxis;
use tiles::TileId;

mod bits;

use crate::WorldPoint;
use bits::CellWalk;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Corridor {
    cells: BTreeSet<TileId>,
    covered: BTreeSet<TileId>,
}

impl Corridor {
    const RIDE_ZOOM: Zoom = Zoom::STATIONS;

    #[must_use]
    pub fn of_trip(trip: &Trip) -> Corridor {
        let legs = || trip.itinerary().days().iter().flat_map(Day::legs);
        let walked = legs().filter(|leg| leg.mode() == Mode::Walk);
        let ridden = legs().filter(|leg| leg.mode() != Mode::Walk);
        let cells: BTreeSet<TileId> = walked
            .flat_map(|leg| CellWalk::cells_along(leg, Zoom::CELL))
            .chain(ridden.flat_map(|leg| CellWalk::cells_along(leg, Self::RIDE_ZOOM)))
            .collect();
        let covered = cells
            .iter()
            .flat_map(|cell| {
                std::iter::successors(Some(*cell), |tile| tile.parent())
                    .take_while(|tile| tile.zoom() >= Zoom::LEAST)
            })
            .collect();
        Corridor { cells, covered }
    }

    #[must_use]
    pub fn holds(&self, tile: TileId) -> bool {
        if tile.zoom() > Zoom::CELL {
            tile.parent()
                .is_some_and(|parent| self.cells.contains(&parent))
        } else {
            self.covered.contains(&tile)
        }
    }

    #[must_use]
    pub fn outline(&self) -> CorridorOutline {
        CorridorOutline(
            self.cells
                .iter()
                .filter(|cell| {
                    !std::iter::successors(cell.parent(), |tile| tile.parent())
                        .any(|parent| self.cells.contains(&parent))
                })
                .copied()
                .collect(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CorridorOutline(Vec<TileId>);

impl fmt::Display for CorridorOutline {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(
            r#"{"type":"Feature","properties":{},"geometry":{"type":"MultiPolygon","coordinates":["#,
        )?;
        self.0.iter().enumerate().try_for_each(|(index, cell)| {
            let beyond = TileId::new(
                cell.zoom(),
                cell.index_along(GridAxis::Across) + 1,
                cell.index_along(GridAxis::Down) + 1,
            );
            let [first, second, third, fourth, last] = WorldPoint::of_tile_corner(*cell)
                .lon_lat()
                .square_to(WorldPoint::of_tile_corner(beyond).lon_lat());
            let lead = if index == 0 { "" } else { "," };
            write!(
                formatter,
                "{lead}[[{first},{second},{third},{fourth},{last}]]"
            )
        })?;
        formatter.write_str("]}}")
    }
}
