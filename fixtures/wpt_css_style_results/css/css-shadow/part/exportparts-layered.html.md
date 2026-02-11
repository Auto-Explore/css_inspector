# css/css-shadow/part/exportparts-layered.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/exportparts-layered.html"
}
```

## style[0]

```css

my-parent::part(base) { font-size: 1px !important; }
my-child::part(base) { font-size: 1px !important; }
my-parent::part(child-base) { font-size: 2rem; }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

      :host::part(base) { background: red !important; }
      my-child::part(child-base) { background: red !important; }
      :host::part(child-base) { background: green; }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css

        :host::part(base) { background: red; }
        :host::part(child-base) { background: red !important; }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
