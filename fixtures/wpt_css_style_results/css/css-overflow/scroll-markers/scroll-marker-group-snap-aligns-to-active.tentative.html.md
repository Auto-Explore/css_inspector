# css/css-overflow/scroll-markers/scroll-marker-group-snap-aligns-to-active.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-snap-aligns-to-active.tentative.html"
}
```

## style[0]

```css

    * {
      margin: 0px;
    }
    .scroller {
      height: 500px;
      width: 80vw;
      border: solid 1px;
      overflow-x: scroll;
      justify-self: center;
      white-space: nowrap;
      padding-top: 50px;
      scroll-snap-type: x mandatory;
      scroll-marker-group: after;
      &::scroll-marker-group {
        border: solid 1px black;
        height: 40px;
        width: 500px;
        display: grid;
        scroll-snap-type: x mandatory;
        grid-auto-flow: column;
        justify-self: center;
        overflow-x: scroll;
        white-space: nowrap;
      }
      counter-set: markeridx -1;
    }

    .box {
      height: 400px;
      width: 50%;
      border: 1px solid;
      display: inline-block;
      margin-left: 100px;
      margin-right: 100px;
      scroll-snap-align: center;
      align-content: center;
      text-align: center;
      &::scroll-marker {
        content: counter(markeridx);
        counter-increment: markeridx;
        width: 20px;
        height: 20px;
        background-color: red;
        margin-right: 100px;
        scroll-snap-align: center;
      }
      &::scroll-marker:target-current {
        background-color: green;
      }
    }

    
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “scroll-snap-type”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “scroll-snap-type”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “scroll-marker-group”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “counter-set”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
