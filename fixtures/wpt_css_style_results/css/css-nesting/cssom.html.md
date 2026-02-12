# css/css-nesting/cssom.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/cssom.html"
}
```

## style[0]

```css

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

        .a {
          & { z-index:1; }
          & #inner1 { z-index:1; }
          .stuff, :is(&) #inner2 { z-index:1; }
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

        div {
          z-index: 1;
          &.test { foo:bar; }
        }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
