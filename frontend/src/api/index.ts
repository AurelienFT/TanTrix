const API_URL = 'http://127.0.0.1:8000';

export function submit() {
    return fetch(`${API_URL}/submit`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify([])
    });
}

export function getDailyGame() {
    return fetch(`${API_URL}/getDailyGame`, {
        method: 'POST',
    });
}