# css/CSS2/tables/border-conflict-style-093.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-conflict-style-093.xht"
}
```

## style[0]

```css

            #table1
            {
                margin-bottom: 20px;
            }
            table
            {
                border-collapse: collapse;
            }
            td
            {
                border: 5px solid lime;
                height: 3em;
                width: 3em;
            }
            #center, #table1 td
            {
                border: 5px double lime;
            }
            #top
            {
                border-top: 5px solid red;
            }
            #left
            {
                border-left: 5px solid red;
            }
            #bottom
            {
                border-bottom: 5px solid red;
            }
            #right
            {
                border-right: 5px solid red;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
