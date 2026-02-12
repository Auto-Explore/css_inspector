# css/css-scroll-snap/snap-events/snap-events-with-pseudo-target.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-events/snap-events-with-pseudo-target.tentative.html"
}
```

## style[0]

```css

      .scroller {
        overflow: scroll;
        width: 200px;
        height: 200px;
        border: solid 1px black;
        scroll-snap-type: y mandatory;
        position: absolute;
        resize: both;
      }
      .space {
        height: 300vh;
        width: 300vw;
        position: absolute;
      }
      .scroller::before, .scroller::after {
        scroll-snap-align: start;
        height: 50px;
        width: 50px;
        color: white;
        display: block;
      }
      .scroller::before {
        content: "before";
        background-color: blue;
      }
      .scroller::after {
        content: "after";
        background-color: orange;
        margin-top: 100px;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
