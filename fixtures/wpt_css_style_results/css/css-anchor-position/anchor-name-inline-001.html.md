# css/css-anchor-position/anchor-name-inline-001.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-name-inline-001.html"
}
```

## style[0]

```css

.container {
  font-family: Ahem;
  font-size: 10px;
  line-height: 1;
  width: 10em;
}
.relpos {
  position: relative;
}
.abspos {
  position: absolute;
}
.anchor {
  anchor-name: --a1;
  background: orange;
}
.target {
  position: absolute;
  width: anchor-size(--a1 width);
  height: 10px;
  background: lime;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
