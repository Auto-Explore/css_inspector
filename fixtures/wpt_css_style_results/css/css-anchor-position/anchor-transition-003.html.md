# css/css-anchor-position/anchor-transition-003.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-transition-003.html"
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

#host.after::part(target) {
  left: anchor(--a left);
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

    div {
      width: 100px;
      height: 100px;
      background: orange;
    }
    #anchor2 {
      margin-left: 200px;
      anchor-name: --a;
    }
    #anchor3 {
      margin-left: 400px;
    }
    #target {
      position: fixed;
      background: lime;
      top: 300px;
      transition: left 10s -5s linear;
    }
    #target.after {
      left: anchor(--a left);
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[2]

```css

    :host { anchor-name: --a; }
    ::slotted(*) { left: anchor(--a left); }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
