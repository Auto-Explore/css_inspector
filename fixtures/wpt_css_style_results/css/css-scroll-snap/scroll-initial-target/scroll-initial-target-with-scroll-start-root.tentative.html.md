# css/css-scroll-snap/scroll-initial-target/scroll-initial-target-with-scroll-start-root.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scroll-initial-target/scroll-initial-target-with-scroll-start-root.tentative.html"
}
```

## style[0]

```css

    :root {
      scroll-start: end end;
    }

    .spacer {
      width: 200vw;
      height: 200vh;
    }

    .box {
      position: absolute;
      width: 60vw;
      height: 60vh;
    }

    .top_left {
      top: 0px;
      left: 0px;
      background-color: red;
    }

    .center {
      top: 60vh;
      left: 60vw;
      background-color: purple;
      scroll-initial-target: nearest;
    }

    .bottom_right {
      top: 120vh;
      left: 120vw;
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
