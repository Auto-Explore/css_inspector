# css/css-animations/animation-name-in-shadow-part-inner-match.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-name-in-shadow-part-inner-match.html"
}
```

## style[0]

```css


#shadow {
  width: 100px;
  height: 100px;
}

@keyframes animation {
  from, to { background-color: blue }
}

#shadow::part(target) {
  height: 100px;
  width: 100px;
  background-color: red;
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

      @keyframes animation {
        from, to { background-color: green }
      }

      div {
        animation: animation 1s both;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
