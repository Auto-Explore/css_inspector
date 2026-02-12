# css/css-scroll-snap/unrelated-gesture-scroll-during-snap.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/unrelated-gesture-scroll-during-snap.html"
}
```

## style[0]

```css

      .scroller {
        border: solid 1px black;
        overflow-y: scroll;
        height: 200px;
        width: 200px;
        display: inline-block;
        background-color: yellow;
        position: relative;
      }
      .snapcontainer {
        scroll-snap-type: y mandatory;
      }
      .snaparea {
        scroll-snap-align: start;
        margin-bottom: 120%;
        height: 40px;
        width: 50px;
        background-color: green;
      }
      .space {
        height: 500vh;
        width: 500vw;
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
