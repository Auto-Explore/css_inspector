# css/css-overflow/scroll-markers/scroll-marker-selection-picks-closest.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-selection-picks-closest.html"
}
```

## style[0]

```css

    .wrapper {
      display: grid;
      justify-content: center;
    }

    .carousel {
      display: grid;
      grid-auto-flow: column;
      width: 512px;
      height: 512px;
      overflow-x: scroll;
      list-style-type: none;
      border: solid 2px grey;
      padding-top: 10%;
      text-align: center;
      counter-set: markeridx -1;

        &>.item {
          height: 80%;
          width: 510px;
          border: 1px solid;
          place-content: center;

          &::scroll-marker {
            content: counter(markeridx);
            counter-increment: markeridx;
            align-content: center;
            text-align: center;
            width: 35px;
            height: 35px;
            border: 3px solid gray;
            border-radius: 50%;
            margin: 3px;
            background-color: red;
          }

          &::scroll-marker:target-current {
            background-color: green;
          }
       }

      scroll-marker-group: after;

      &::scroll-marker-group {
        height: 45px;
        display: flex;
        align-items: center;
        justify-content: center;
        border: solid 1px black;
        border-radius: 30px;
      }
    }
  
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “counter-set”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “scroll-marker-group”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
