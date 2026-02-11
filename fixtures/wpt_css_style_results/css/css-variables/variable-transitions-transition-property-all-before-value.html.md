# css/css-variables/variable-transitions-transition-property-all-before-value.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-transitions-transition-property-all-before-value.html"
}
```

## style[0]

```css

        #target
        {
            transition-property: all;
            transition-duration: 1s;
        }
        #target
        {
            --value: blue;
            color: var(--value);
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
