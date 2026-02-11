# css/css-anchor-position/anchor-scope-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scope-dynamic.html"
}
```

## style[0]

```css

  .scope-all { anchor-scope: all; }
  .scope-a { anchor-scope: --a; }
  .scope-b { anchor-scope: --b; }
  .scope-ab { anchor-scope: --a, --b; }

  .anchor-a { anchor-name: --a; }
  .anchor-b { anchor-name: --b; }
  .anchor-a, .anchor-b {
    background: skyblue;
    height: 10px;
  }

  .anchored-a { position-anchor: --a; }
  .anchored-b { position-anchor: --b; }
  .anchored-a, .anchored-b {
    position: absolute;
    top: anchor(bottom);
    left: anchor(left);
    width: 5px;
    height: 5px;
    background: coral;
  }

  /* Containing block */
  main {
    position: relative;
    width: 100px;
    height: 100px;
    border: 1px solid black;
  }
```

```json
{
  "errors": 10,
  "messages": [
    {
      "message": "Unknown property “anchor-scope”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-scope”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-scope”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-scope”.",
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
