# css/css-scroll-anchoring/focused-element-in-excluded-subtree.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-anchoring/focused-element-in-excluded-subtree.html"
}
```

## style[0]

```css


body { height: 4000px; }
div { height: 100px; }

.scroller {
  overflow: scroll;
  position: fixed;
  width: 300px;
  height: 300px;
  background-color: green;
}

#posSticky {
  top: 300px;
  position: relative;
  height: 50px;
  width: 50px;
  background-color: blue;
}

#content {
  background-color: #D3D3D3;
  height: 50px;
  width: 50px;
  position: relative;
  top: 500px;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
