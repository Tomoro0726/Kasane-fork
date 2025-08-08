use logic::{
    id::{DimensionRange, SpaceTimeId},
    set::SpaceTimeIdSet,
};

fn main() {
    //時空間IDを作成してくれる
    let id = SpaceTimeId::new(
        3,
        DimensionRange::Single(1),
        DimensionRange::LimitRange(3, 7),
        DimensionRange::Any,
        0,
        DimensionRange::Any,
    )
    .unwrap();

    //時空間IDの集合を作成する
    let mut set = SpaceTimeIdSet::new();

    //集合にIDを入れる
    set.insert(id);

    //別の時空間IDを作成する
    let id2 = SpaceTimeId::new(
        3,
        DimensionRange::LimitRange(1, 5),
        DimensionRange::LimitRange(3, 5),
        DimensionRange::Any,
        0,
        DimensionRange::Any,
    )
    .unwrap();

    //別の時空間IDの集合を作成する
    let mut set2 = SpaceTimeIdSet::new();

    //集合にIDを入れる
    set2.insert(id2);

    //2つの時空間IDの集合を演算する（AND）
    let set3 = set & set2;

    //演算の結果を出力する
    println!("{}", set3);

    //演算の結果を拡張記法を用いることなく出力する
    println!("{:?}", set3);

    //NOTの使い方
    let not = !set3;
    println!("{}", not);
}
