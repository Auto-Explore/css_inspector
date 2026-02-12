# css/css-anchor-position/anchor-name-in-shadow-002.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-name-in-shadow-002.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

.host {
  width: 100px;
  height: 100px;
}

#host2 {
  margin-left: 200px;
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

      div { width: 100px; height: 100px; }
      #anchor { anchor-name: --a; background: orange; }
      #target {
        position: fixed;
        background: lime;
        left: anchor(--a left);
        top: anchor(--a bottom);
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
