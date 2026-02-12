# css/css-anchor-position/anchor-transition-002.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-transition-002.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

#anchor1 {
  width: 100px;
  height: 100px;
  background: orange;
  anchor-name: --a;
}

#anchor2 {
  width: 100px;
  height: 100px;
  margin-top: 200px;
  margin-left: 300px;
  background: orange;
}

#anchor2.after::part(target) {
  top: anchor(--a top);
  left: anchor(--a right);
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
      #target {
        position: fixed;
        width: 100px;
        height: 100px;
        background: lime;
        top: anchor(--a top);
        left: anchor(--a right);
        transition: all 10s -5s linear;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
