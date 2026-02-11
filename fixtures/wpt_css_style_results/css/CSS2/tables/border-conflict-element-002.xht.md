# css/CSS2/tables/border-conflict-element-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-conflict-element-002.xht"
}
```

## style[0]

```css

            table
            {
                border-collapse: collapse;
                direction: rtl;
                height: 2in;
                width: 2in;
            }
            td
            {
                border: 5px solid black;
                direction: ltr;
            }
            .collapsing1
            {
                border-right-color: red;
            }
            #collapsing2
            {
                border-top-color: red;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
