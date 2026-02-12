# css/css-values/attr-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-values/attr-crash.html"
}
```

## style[0]

```css

        #div {
            --prop: attr(data-foo type(<ident>));
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
