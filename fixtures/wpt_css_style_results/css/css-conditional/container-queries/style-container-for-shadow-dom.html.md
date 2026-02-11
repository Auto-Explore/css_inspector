# css/css-conditional/container-queries/style-container-for-shadow-dom.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/style-container-for-shadow-dom.html"
}
```

## style[0]

```css

        @container style(--foo: bar) {
          #t1 { color: green; }
        }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

      @container style(--foo: baz) {
        #t2 { color: green; }
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[2]

```css

        @container style(--foo: bar) {
          ::slotted(#t3) { color: green; }
        }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[3]

```css

        @container style(--foo: bar) {
          :host(#t4) { color: green; }
        }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[4]

```css

      @container style(--foo: baz) {
        #ancestor-part > div::part(part) { color: green; }
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[5]

```css

        @container style(--foo: baz) {
          ::slotted(#t6)::before {
            content: "X";
            color: green;
          }
        }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[6]

```css

        @container style(--foo: bar) {
          :host(#t7)::before {
            content: "X";
            color: green;
          }
        }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[7]

```css

    @container style(--foo: quz) {
      #ancestor-part-before > div::part(part)::before {
        content: "X";
        color: green;
      }
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[8]

```css

    @container style(--foo: quz) {
      #ancestor-inner-part > div::part(inner-part) { color: green; }
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[9]

```css

      @container style(--foo: bar) {
        #t10 { color: green; }
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[10]

```css

      @container style(--foo: bar) {
        #container-for-part > div::part(part) { color: green; }
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[11]

```css

        @container style(--foo: baz) {
          :host::part(part) { color: green; }
        }
      
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
