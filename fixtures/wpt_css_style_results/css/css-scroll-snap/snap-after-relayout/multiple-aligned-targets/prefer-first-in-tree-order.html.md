# css/css-scroll-snap/snap-after-relayout/multiple-aligned-targets/prefer-first-in-tree-order.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-after-relayout/multiple-aligned-targets/prefer-first-in-tree-order.html"
}
```

## style[0]

```css

    .large-space {
      position: absolute;
      height: 300vh;
      width: 300vw;
      z-index: -1;
    }
    .target {
      width: 100px;
      height: 100px;
      background-color: green;
      scroll-snap-align: start;
    }
    .snapcontainer {
      border:solid 1px black;
      overflow: scroll;
      scroll-snap-type: y mandatory;
    }
    .outer {
      height: 315px;
      width: 1200px;
      position: relative;
    }
    .inner {
      height: 115px;
      width: 120px;
    }
    .positioned {
      position: absolute;
      top: 150px;
    }
    .outer .target1 {
      left: 0px;
    }
    .outer .target2 {
      left: 110px;
    }
    .outer .target3 {
      left: 220px;
    }
    .outer .target4 {
      left: 330px;
    }
    .outer .target5 {
      left: 440px;
    }
    .inner .target1 {
      left: 550px;
    }
    .inner .target2 {
      left: 660px;
    }
    .inner .target3 {
      left: 770px;
    }
    .inner .target4 {
      left: 880px;
    }
    .inner .target5 {
      left: 990px;
    }
    .placeholder {
      background-color: purple;
      top: 0px;
    }
    .outer > .placeholder {
      position: absolute;
      top: 0px;
      left: 200px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
