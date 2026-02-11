# css/CSS2/selectors/attribute-token-selector-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/selectors/attribute-token-selector-002.xht"
}
```

## style[0]

```css

    p {
      color: green;
    }

    p.valid {
      color: red;
    }
    [title~=""], [title~=''], p.valid {
      color: green;
    }
    [title~=], p.valid {
      color: red;
    }

    [title~=""] {
      color: red;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
