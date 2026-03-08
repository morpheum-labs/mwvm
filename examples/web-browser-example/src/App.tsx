import { useState, useEffect } from 'react';
import initMwvm, {
    McpToolCall,
    toolsListRequest,
    hexToBytes,
    bytesToHex,
} from 'mwvm-wasm';

type Status = 'loading' | 'ready' | 'running' | 'error';

function App() {
    const [status, setStatus] = useState<Status>('loading');
    const [output, setOutput] = useState<string>('');
    const [error, setError] = useState<string>('');

    // Initialise the WASM module on mount
    useEffect(() => {
        const init = async () => {
            try {
                await initMwvm();
                setStatus('ready');
                setOutput('WASM module loaded. Ready to go!');
            } catch (err) {
                console.error(err);
                setStatus('error');
                setError(`Failed to initialise WASM module: ${err}`);
            }
        };
        init();
    }, []);

    // Demo: build an MCP tool call and show the JSON-RPC body
    const runMcpDemo = () => {
        setStatus('running');
        setOutput('');
        setError('');

        try {
            // Build a tool call for the MWVM gateway
            const call = new McpToolCall(
                'morpheum_infer',
                JSON.stringify({ max_tokens: 512, model: 'llama-3-8b' }),
            );

            const jsonRpc = call.toJsonRpc();

            // Also demonstrate tools/list request
            const listReq = toolsListRequest();

            setOutput(
                `── McpToolCall ──\n` +
                `Name:      ${call.name}\n` +
                `Args JSON: ${call.argsJson}\n` +
                `JSON-RPC:  ${jsonRpc}\n\n` +
                `── tools/list ──\n` +
                `${listReq}`,
            );
        } catch (err) {
            setError(`MCP demo failed: ${err}`);
        } finally {
            setStatus('ready');
        }
    };

    // Demo: hex encoding / decoding round-trip
    const runHexDemo = () => {
        setStatus('running');
        setOutput('');
        setError('');

        try {
            const input = 'deadbeef01020304';
            const bytes = hexToBytes(input);
            const roundTripped = bytesToHex(bytes);

            setOutput(
                `── Hex Round-Trip ──\n` +
                `Input:        ${input}\n` +
                `Decoded:      [${Array.from(bytes).join(', ')}]\n` +
                `Re-encoded:   ${roundTripped}\n` +
                `Match:        ${input === roundTripped ? '✅ yes' : '❌ no'}`,
            );
        } catch (err) {
            setError(`Hex demo failed: ${err}`);
        } finally {
            setStatus('ready');
        }
    };

    return (
        <div className="app">
            <h1>MWVM Browser Example</h1>
            <p>Running Morpheum WASM bindings directly in the browser</p>

            <div className="card">
                <h2>Status</h2>
                <p>
                    <strong>
                        {status === 'loading' && '⏳ Loading WASM module...'}
                        {status === 'ready' && '✅ Ready'}
                        {status === 'running' && '⚡ Running...'}
                        {status === 'error' && '❌ Error'}
                    </strong>
                </p>

                {output && (
                    <pre style={{ background: '#1a1a1a', padding: '1rem', borderRadius: '8px', whiteSpace: 'pre-wrap' }}>
                        {output}
                    </pre>
                )}

                {error && <p style={{ color: '#ff6b6b' }}>{error}</p>}

                <div style={{ display: 'flex', gap: '1rem', marginTop: '1rem' }}>
                    <button
                        onClick={runMcpDemo}
                        disabled={status !== 'ready'}
                        style={buttonStyle(status === 'ready')}
                    >
                        MCP Tool Call Demo
                    </button>

                    <button
                        onClick={runHexDemo}
                        disabled={status !== 'ready'}
                        style={buttonStyle(status === 'ready')}
                    >
                        Hex Round-Trip Demo
                    </button>
                </div>
            </div>

            <footer style={{ marginTop: '2rem', textAlign: 'center', opacity: 0.6 }}>
                Built with MWVM • Morpheum WASM Virtual Machine
            </footer>
        </div>
    );
}

function buttonStyle(enabled: boolean): React.CSSProperties {
    return {
        padding: '12px 24px',
        fontSize: '1rem',
        background: enabled ? '#00ff9d' : '#555',
        color: '#000',
        border: 'none',
        borderRadius: '8px',
        cursor: enabled ? 'pointer' : 'not-allowed',
        fontWeight: 600,
    };
}

export default App;
