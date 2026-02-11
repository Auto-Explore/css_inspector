# css/css-anchor-position/pseudo-element-implicit-anchor-center.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/pseudo-element-implicit-anchor-center.html"
}
```

## style[0]

```css

  #anchor {
    display: flow-root;
    margin-left: -50px;
    margin-top: -50px;
    width: 200px;
    height: 200px;
  }
  #anchor::after {
    content: "";
    position: absolute;
    position-anchor: auto;
    width: 100px;
    height: 100px;
    background: green;
    /* Should be centered inside #anchor */
    place-self: anchor-center;
  }
  #ref {
    margin-left: 50px;
    margin-top: 50px;
    width: 100px;
    height: 100px;
    background-color: red;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
