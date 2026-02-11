# css/css-anchor-position/anchor-name-cross-shadow.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-name-cross-shadow.html"
}
```

## style[0]

```css

.cb {
  position: absolute;
}

#host1::part(anchor) {
  anchor-name: --a1;
  margin-left: 15px;
}
#target1 {
  position: absolute;
  left: anchor(--a1 left);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

      :host {
        anchor-name: --a2;
        margin-left: 15px;
      }
      #target2 {
        position: absolute;
        left: anchor(--a2 left);
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
