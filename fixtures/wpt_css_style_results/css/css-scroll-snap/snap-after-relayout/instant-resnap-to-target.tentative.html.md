# css/css-scroll-snap/snap-after-relayout/instant-resnap-to-target.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-after-relayout/instant-resnap-to-target.tentative.html"
}
```

## style[0]

```css

      #scroller {
        height: 200px;
        width: 200px;
        overflow-y: scroll;
        border: 1px;
        scroll-snap-type: y mandatory;
        scroll-behavior: smooth;
      }
      .target {
        scroll-snap-align: start;
        height: 100px;
        width: 100px;
      }
      .pad {
        width: 50%;
        height: 300px;
      }
      #child1 {
        background-color: blue;
      }
      #child2 {
        background-color: green;
      }
    
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
