# css/CSS2/syntax/comments-006.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/comments-006.xht"
}
```

## style[0]

```css

            *
            {
                color: green;
            }
            p
            {
                color: black;
            }
            div
            {
                /*\*/*/color: red;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid property name in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
