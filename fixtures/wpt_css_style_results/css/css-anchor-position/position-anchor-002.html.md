# css/css-anchor-position/position-anchor-002.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-anchor-002.html"
}
```

## style[0]

```css

.anchor {
  width: 100px;
  height: 100px;
  background: orange;
  font: 20px/1 Ahem;
}

.target {
  position: fixed;
  background: lime;
  position-try-fallbacks: --pf;
  left: 999999px; /* force fallback */
  font: 20px/1 Ahem;
}

@position-try --pf {
  top: anchor(bottom, 0px);
  left: anchor(left, 0px);
  width: anchor-size(width, 0px);
  height: anchor-size(height, 0px);
}

body {
  margin: 0;
}

#fake-anchor {
  anchor-name: --a;
}

#anchor1 {
  margin-left: 100px;
}

#anchor2 {
  margin-left: 300px;
  margin-top: 100px;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

      :host { anchor-name: --a; }
      ::slotted(.target) { position-anchor: --a; }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
