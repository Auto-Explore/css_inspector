# css/css-anchor-position/anchor-scope-shadow-all.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scope-shadow-all.html"
}
```

## style[0]

```css

  #host::part(scope) {
    /* This should have no effect, because 'all' is tree-scoped,
       and specified in a different tree-scope than the anchor names
       in the shadow. */
    anchor-scope: all;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-scope”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

      .anchored {
        background: coral;
        position: absolute;
        top: anchor(bottom, 1px);
        position-anchor: --a;
        width: 5px;
        height: 5px;
      }
      .anchor {
        background: skyblue;
        height: 10px;
        anchor-name: --a;
      }
      .cb {
        position: relative;
        width: 200px;
        height: 200px;
        border: 1px solid black;
      }
      .scope {
        anchor-scope: --a;
      }
    
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-scope”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
