method calculateRace(holdTime: int, totalTime: int) returns (distance: int)
  requires holdTime >= 0
  requires totalTime >= 0
  requires holdTime <= totalTime
  ensures distance >= 0
{
  var speed: int := holdTime;
  var movingTime: int := totalTime - holdTime;
  distance := speed * movingTime;
}

method raceOptions(totalTime: int, currentRecord: int) returns (options: int)
  requires totalTime >= 0
  requires currentRecord >= 0
  requires currentRecord <= totalTime
  ensures options >= 0
{
    options := 0;

    var holdTime := 0;
    while holdTime < totalTime {
        var distance := calculateRace(holdTime, totalTime);
        if  distance > currentRecord {
            options := options + 1;
        }
        holdTime := holdTime + 1;
    }
}
