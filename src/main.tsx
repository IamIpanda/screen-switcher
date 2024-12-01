import { render } from 'preact'
import { useState } from 'preact/hooks'
import { Button, Space } from 'antd'

import "./main.css"

const OPTIONS = [
    { label: 'No', value: 0 },
    { label: '1', value: 1 },
    { label: '2', value: 2 },
    { label: '3', value: 3 },
    { label: '4', value: 4 },
    { label: '5', value: 5 },
    { label: '6', value: 6 },
    { label: '7', value: 7 },
    { label: '8', value: 8 },
]

function App() {
    let [switches, setSwitches] = useState([1, 2, 3, 4, 5, 6, 7, 8])
    const c = (output: number, input: number) => {
        switches[output] = input;
        setSwitches([...switches])
        fetch(import.meta.env.BASE_URL + `set?from=${input}&to=${output+1}`, { method: 'PUT' })
    }
    return (
        <div id="app">
            <h1>Screen Switcher</h1>
            {[...Array(8).keys()].map((i) => <div class="segement-line">
                <div class="label">Output {i + 1}</div>
                <Space.Compact>
                    {OPTIONS.map((option) => 
                    <Button type={switches[i] == option.value ? 'primary' : undefined} onClick={() => c(i, option.value)}>{option.label}</Button>)}
                </Space.Compact>
            </div>)}
            <Button className='oneone' danger onClick={() => {
                setSwitches([1,2,3,4,5,6,7,8])
                fetch(import.meta.env.BASE_URL + `reset`, { method: 'PUT' })
            }}>1 to 1</Button>
        </div>
    )
}

render(<App />, document.getElementById('app')!)
