# css/css-scroll-snap/snap-after-relayout/multiple-aligned-targets/prefer-targeted-element-main-frame-target.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-after-relayout/multiple-aligned-targets/prefer-targeted-element-main-frame-target.html"
}
```

## style[0]

```css

    #space {
      height: 300vh;
      width: 300vw;
      position: absolute;
    }
    #scroller {
      overflow-y: scroll;
      scroll-snap-type: y mandatory;
      width: 450px;
      height: 450px;
      border: solid 1px black;
      position: relative;
    }
    .box {
      height: 200px;
      width: 200px;
      position: absolute;
      background-color: green;
      scroll-snap-align: start;
    }
    .box:target {
      background-color: red;
    }
    .toprow { top: 0px; }
    .midrow { top: 210px; }
    .bottomrow { top: 420px; }
    .leftcol { left: 0px; }
    .midcol { left: 210px; }
    .rightcol { left: 420px; }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “scroll-snap-type”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
