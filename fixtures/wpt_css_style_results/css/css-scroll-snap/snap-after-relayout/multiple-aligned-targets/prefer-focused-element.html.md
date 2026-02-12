# css/css-scroll-snap/snap-after-relayout/multiple-aligned-targets/prefer-focused-element.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-after-relayout/multiple-aligned-targets/prefer-focused-element.html"
}
```

## style[0]

```css

      .scroller {
        overflow: scroll;
        position: relative;
        height: 400px;
        width: 400px;
        border:solid 1px black;
        scroll-snap-type: y mandatory;
      }
      .no-snap { scroll-snap-align: none }
      .scroller div:focus {
        border: solid 1px red;
      }
      .large-space {
        height: 300vh;
        width: 300vw;
      }
      .target {
        scroll-snap-align: start;
        position: absolute;
        width: 100px;
        height: 100px;
        border: solid 1px black;
      }
      .top {
        top: 0px;
      }
      .left {
        left: 0px;
      }
      .right {
        left: 200px;
      }
      .bottom {
        top: 200px;
      }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
