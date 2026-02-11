# css/css-syntax/declarations-trim-whitespace.html

```json
{
  "format_version": 3,
  "file": "css/css-syntax/declarations-trim-whitespace.html"
}
```

## style[0]

```css

    #foo {
        --foo-1:bar;
        --foo-2: bar;
        --foo-3:bar ;
        --foo-4: bar ;
        --foo-5: bar !important;
        --foo-6: bar !important ;
        --foo-7:bar!important;
        --foo-8:bar!important ;
        --foo-9:bar
    }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
