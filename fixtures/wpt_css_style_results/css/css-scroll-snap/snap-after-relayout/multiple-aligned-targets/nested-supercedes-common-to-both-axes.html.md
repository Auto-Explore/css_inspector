# css/css-scroll-snap/snap-after-relayout/multiple-aligned-targets/nested-supercedes-common-to-both-axes.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-after-relayout/multiple-aligned-targets/nested-supercedes-common-to-both-axes.html"
}
```

## style[0]

```css

      .scroller {
        overflow: scroll;
        width: 450px;
        height: 450px;
        border: solid 1px black;
        scroll-snap-type: both mandatory;
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
        width: 200px;
        height: 200px;
        background-color: green;
        position: absolute;
      }
      .inner {
        width: 50px;
        height: 50px;
        background-color: yellow;
      }
      #box2 {
        top: 0px;
        left: 100px;
      }
      #box3 {
        top: 100px;
        left: 0px;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
