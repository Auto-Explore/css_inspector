# css/css-cascade/scope-part.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-part.html"
}
```

## style[0]

```css

    :host {
      display: block;
    }
    div {
      padding: 5px;
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

    :host {
      display: block;
    }
    div {
      padding: 5px;
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

    @scope {
      :scope {
        background: red;
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

## style[3]

```css

    @scope {
      :scope::part(a), :scope::part(b) {
        background: blue;
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

## style[4]

```css

    @scope {
      :scope::part(a), :scope::part(b) {
        background: red;
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

## style[5]

```css

      @scope {
        :scope::part(a), :scope::part(b) {
          background: blue;
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

    @scope {
      :scope::part(a) {
        background: blue;
      }
      :scope::part(b) {
        background: red;
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

        @scope {
          :scope::part(a) {
            background: blue;
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

## style[8]

```css

        @scope {
          :scope {
            background: red;
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
