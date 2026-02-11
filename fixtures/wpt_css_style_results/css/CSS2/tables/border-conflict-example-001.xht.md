# css/CSS2/tables/border-conflict-example-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-conflict-example-001.xht"
}
```

## style[0]

```css

            table
            {
                border-collapse: collapse;
                border: 5px solid yellow;
            }
            *#col1
            {
                border: 3px solid black;
            }
            td
            {
                border: 1px solid red;
                padding: 1em;
                width: 2em;
                height: 2em;
            }
            td.cell5
            {
                border: 5px dashed blue;
            }
            td.cell6
            {
                border: 5px solid lime;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
