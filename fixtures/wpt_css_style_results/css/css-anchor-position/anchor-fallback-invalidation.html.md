# css/css-anchor-position/anchor-fallback-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-fallback-invalidation.html"
}
```

## style[0]

```css

  #cb {
    position: relative;
    width: 200px;
    height: 200px;
    border: 1px solid black;
  }

  #anchor {
    anchor-name: --a;
    position: absolute;
    width: 40px;
    height: 30px;
    left: 75px;
    top: 75px;
    background: coral;
  }

  #anchored {
    position: absolute;
    background: seagreen;
    width: 50px;
    height: 50px;
  }

  #anchored.change {
    /* The fallbacks match what the unchanged style says, but we shouldn't
       take the fallbacks here. */
    width: anchor-size(--a width, 50px);
    height: anchor-size(--a height, 50px);
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
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
