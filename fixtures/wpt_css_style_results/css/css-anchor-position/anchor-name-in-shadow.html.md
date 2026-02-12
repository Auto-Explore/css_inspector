# css/css-anchor-position/anchor-name-in-shadow.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-name-in-shadow.html"
}
```

## style[0]

```css

  body { margin-top: 0; }
  #anchor {
    anchor-name: --anchor;
  }
  #filler {
    height: 100px;
  }
  #anchored {
    position: absolute;
    top: anchor(--anchor top);
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

      div { anchor-name: --anchor; }
    
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

      div {
        position: absolute;
        left: anchor(--anchor-host left, 37px);
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
