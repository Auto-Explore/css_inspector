# css/css-overflow/scroll-markers/targeted-column-scroll-marker-selection-002.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/targeted-column-scroll-marker-selection-002.html"
}
```

## style[0]

```css

    .wrapper {
      display: grid;
      justify-content: center;
    }

    .carousel {
      width: 800px;
      height: 200px;
      overflow-x: scroll;
      scroll-snap-type: x mandatory;
      list-style-type: none;
      scroll-behavior: smooth;
      border: solid 2px grey;
      padding-top: 10%;
      text-align: center;
      counter-set: markeridx -1;
      columns: 2;

      &::column::scroll-marker {
        content: counter(markeridx);
        counter-increment: markeridx;
        align-content: center;
        text-align: center;
        width: 35px;
        height: 35px;
        border-radius: 50%;
        margin: 3px;
        background-color: red;
      }
      &::column::scroll-marker:target-current {
        background-color: green;
      }

      &>.item {
        scroll-snap-align: center;
        height: 80%;
        width: 158px;
        border: 1px solid;
        place-content: center;
        display: inline-block;
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
  "errors": 7,
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
      "message": "Invalid value for property “scroll-snap-type”.",
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
