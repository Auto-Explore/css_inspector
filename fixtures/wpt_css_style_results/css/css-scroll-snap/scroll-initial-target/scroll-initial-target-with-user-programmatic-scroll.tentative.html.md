# css/css-scroll-snap/scroll-initial-target/scroll-initial-target-with-user-programmatic-scroll.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scroll-initial-target/scroll-initial-target-with-user-programmatic-scroll.tentative.html"
}
```

## style[0]

```css

    .spacer {
      width: 1000px;
      height: 1000px;
    }

    .scroller {
      width: 300px;
      height: 300px;
      border: solid 1px black;
      overflow: scroll;
      margin: 0px;
      position: absolute;
    }

    .box {
      position: absolute;
      width: 200px;
      height: 200px;
    }

    .top_left {
      top: 0px;
      left: 0px;
      background-color: red;
    }

    .center {
      top: 200px;
      left: 200px;
      background-color: purple;
      scroll-initial-target: nearest;
    }

    .bottom_right {
      top: 400px;
      left: 400px;
      background-color: yellow;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
