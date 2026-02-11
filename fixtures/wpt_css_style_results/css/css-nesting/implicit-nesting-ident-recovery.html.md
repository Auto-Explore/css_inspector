# css/css-nesting/implicit-nesting-ident-recovery.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/implicit-nesting-ident-recovery.html"
}
```

## style[0]

```css

  #target1 {
    display:block;
    display:new-block;
    color:green;
  }
  #target2 {
    display:block;
    display:hover {};
    color:green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
