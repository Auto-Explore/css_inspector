# css/css-scroll-snap/snap-after-relayout/layout-follows-focused-targeted-block-iframe.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-after-relayout/layout-follows-focused-targeted-block-iframe.html"
}
```

## style[0]

```css

      .scroller {
        width: 200px;
        height: 200px;
        border: solid 1px black;
        overflow: scroll;
        scroll-snap-type: both mandatory;
        position: relative;
        resize: both;
      }
      .target {
        scroll-snap-align: start;
        width: 50px;
        height: 50px;
        background-color: green;
        position: absolute;
      }
      .target:target {
        background-color: blue;
      }
      .target:focus {
        background-color: yellow;
      }
      #box1 {
        left: 150px;
        top: 0px;
      }
      #box2 {
        left: 0px;
        top: 150px;
      }
      .space {
        width: 500%;
        height: 500%;
        position: absolute;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
