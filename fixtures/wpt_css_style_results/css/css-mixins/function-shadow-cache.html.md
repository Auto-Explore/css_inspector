# css/css-mixins/function-shadow-cache.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/function-shadow-cache.html"
}
```

## style[0]

```css

  @function --a() { result: 42px; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

      @function --a() {
        result: 10px;
      }
      div {
        width: --a();
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css

      div {
        width: --a();
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
