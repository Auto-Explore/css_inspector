# css/css-writing-modes/dynamic-offset-vrl-002.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/dynamic-offset-vrl-002.html"
}
```

## style[0]

```css


html {
  writing-mode: vertical-rl;
  overflow: hidden;
}

#container {
  position: absolute;
  top: 0;
  left: 0;
  width: 200px;
  height: 100px;
  background: yellow;
}

#abspos {
  position: absolute;
  top: 10px;
  right: 20px;
  left: 10px; /* ignored */
  width: 10px;
  height: 10px;
  background: fuchsia;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
