# css/css-anchor-position/anchor-scroll-position-try-013.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-position-try-013.html"
}
```

## style[0]

```css

#cb {
  position: relative;
  width: 200px;
  height: 200px;
  border: solid 1px;
}

#scroller {
  width: 200px;
  height: 200px;
  overflow: scroll;
}

#abspos {
  position: absolute;
  background: hotpink;
  width: 50px;
  height: 50px;

  position-area: bottom;
  position-try-fallbacks: flip-block;
  position-anchor: --a;
}

#anchor {
  anchor-name: --a;
  width: 50px;
  height: 50px;
  margin: 150px 0 150px 50px;
  background: dodgerblue;
}

```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-area”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
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
