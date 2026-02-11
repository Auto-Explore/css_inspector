# css/css-anchor-position/anchor-ident-function.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-ident-function.html"
}
```

## style[0]

```css

  #cb {
    border: 1px solid black;
    width: 100px;
    height: 100px;
    position: relative;
  }
  .anchor {
    width: 15px;
    height: 15px;
    background-color: skyblue;
    position: absolute;
    top: 30px;
  }
  /* sibling-index() could make this nicer, but it's not universally
     supported at the time of writing: */
  .anchor:nth-child(1) { anchor-name: --a1; left: calc(20px * 1); }
  .anchor:nth-child(2) { anchor-name: --a2; left: calc(20px * 2); }
  .anchor:nth-child(3) { anchor-name: --a3; left: calc(20px * 3); }
  #target {
    width: 15px;
    height: 15px;
    background-color: tomato;
    position: absolute;
    position-anchor: --a1;
  }
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
