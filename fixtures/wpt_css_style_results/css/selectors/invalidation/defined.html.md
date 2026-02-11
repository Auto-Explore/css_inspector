# css/selectors/invalidation/defined.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/defined.html"
}
```

## style[0]

```css

      #container {
        color: gray;
      }

      #a1:defined {
        color: blue;
      }
      :defined + #b1 {
        color: green;
      }
      :defined > #c1 {
        color: red;
      }
      div + :defined + * #d1 {
        color: yellow;
      }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
