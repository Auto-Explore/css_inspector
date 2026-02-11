# css/CSS2/tables/border-conflict-element-012.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-conflict-element-012.xht"
}
```

## style[0]

```css

            table
            {
                border-collapse: collapse;
                height: 2in;
                width: 2in;
            }
            #test
            {
                border-bottom: 5px solid red;
                border-top: 5px solid red;
            }
            td
            {
                border-left: 5px solid black;
                border-right: 5px solid black;
            }
            .winning
            {
                border-bottom: 5px solid black;
                border-top: 5px solid black;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
