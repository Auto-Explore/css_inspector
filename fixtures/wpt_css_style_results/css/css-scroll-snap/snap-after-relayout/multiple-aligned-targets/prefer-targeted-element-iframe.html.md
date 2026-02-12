# css/css-scroll-snap/snap-after-relayout/multiple-aligned-targets/prefer-targeted-element-iframe.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-after-relayout/multiple-aligned-targets/prefer-targeted-element-iframe.html"
}
```

## style[0]

```css

      .scroller {
        overflow: scroll;
        width: 350px;
        height: 350px;
        border: solid 1px black;
        scroll-snap-type: y mandatory;
        position: relative;
        resize: both;
      }
      .large-space {
        height: 300vh;
        width: 300vw;
        position: absolute;
      }
      .snap {
        scroll-snap-align: start;
      }
      .box {
        width: 100px;
        height: 100px;
        background-color: green;
        display: inline-block;
        position: relative;
      }
      .grid {
        position: absolute;
        width: 350px;
        height: 350px;
      }
      .snap:target {
        background-color: blue;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
