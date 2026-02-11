# css/css-mixins/function-shadow.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/function-shadow.html"
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
      #target {
        --actual: --a();
        --expected: 10px;
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

      #target {
        --actual: --a();
        --expected: 42px;
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

          @function --a() {
            result: 11px;
          }
          #target {
            --actual: --a();
            --expected: 11px;
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

## style[4]

```css

      @function --a() {
        result: 12px;
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

## style[5]

```css

          #target {
            --actual: --a();
            --expected: 12px;
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

      @function --b() {
        result: B;
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

## style[7]

```css

          @function --c() {
            result: C;
          }
          #target {
            --actual: --a() --b() --c();
            --expected: 42px B C;
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

## style[8]

```css

      @function --b() {
        result: 14px;
      }
      ::part(target) {
        --actual: --a() --b();
        --expected: 42px 14px;
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

## style[9]

```css

          @function --a() {
            result: FAIL-a;
          }
          @function --b() {
            result: FAIL-b;
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

## style[10]

```css

      @function --b() {
        result: 16px;
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

## style[11]

```css

          @function --c() {
            result: 15px;
          }
          ::slotted(#target) {
            --actual: --a() --b() --c();
            --expected: 42px 16px 15px;
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

## style[12]

```css

      @function --b() {
        result: 17px;
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

## style[13]

```css

          @function --c() {
            result: 18px;
          }
          :host {
            --actual: --a() --b() --c();
            --expected: 42px 17px 18px;
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

## style[14]

```css

      @function --b() {
        result: --c(); /* 20px */
      }
      @function --c() {
        result: 20px;
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

## style[15]

```css

          @function --c() {
            result: C;
          }
          #target {
            --actual: --b() --c();
            --expected: 20px C;
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

## style[16]

```css

      @function --b() {
        --lb: --c();
        result: var(--lb); /* 22px */
      }
      @function --c() {
        result: 22px;
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

## style[17]

```css

          @function --c() {
            result: C;
          }
          @function --d() {
            --ld: --b() --c();
            result: var(--ld);
          }
          #target {
            --actual: --d();
            --expected: 22px C;
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

## style[18]

```css

      @function --a() {
        result: 24px;
      }
      @function --b() {
        result: --a(); /* 24px */
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

## style[19]

```css

          @function --a() {
            result: --b(); /* Calls --b() in outer scope. */
          }
          #target {
            /* Nothing is in a cycle here. */
            --actual: --a();
            --expected: 24px;
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
