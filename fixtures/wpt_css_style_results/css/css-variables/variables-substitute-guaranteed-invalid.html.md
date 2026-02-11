# css/css-variables/variables-substitute-guaranteed-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variables-substitute-guaranteed-invalid.html"
}
```

## style[0]

```css

    #target1 {
        /* Cycle */
        --var1: var(--var2);
        --var2: var(--var1);

        /* Reference to cycle */
        --var3: var(--var1);

        /* Reference to non-existent property */
        --var4: var(--noexist);
    }

    #target1parent {
        --var3: inherited;
        --var4: inherited;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
