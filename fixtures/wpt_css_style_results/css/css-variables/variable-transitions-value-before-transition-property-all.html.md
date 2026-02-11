# css/css-variables/variable-transitions-value-before-transition-property-all.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-transitions-value-before-transition-property-all.html"
}
```

## style[0]

```css

        #target
        {
            --value: blue;
            color: var(--value);
        }
        #target
        {
            transition-property: all;
            transition-duration: 1s;
        }
        #target.changed
        {
            --value: green;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
