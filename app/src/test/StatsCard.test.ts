import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import StatsCard from '../components/dashboard/StatsCard.svelte';

describe('StatsCard', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders title and value correctly', () => {
    render(StatsCard, {
      props: {
        title: 'CPU Usage',
        value: 45.5,
        unit: '%',
        icon: 'cpu',
        color: 'blue',
      },
    });

    expect(screen.getByText('CPU Usage')).toBeInTheDocument();
    expect(screen.getByText('45.5%')).toBeInTheDocument();
  });

  it('displays trend indicator when provided', () => {
    render(StatsCard, {
      props: {
        title: 'Memory',
        value: 8.2,
        unit: 'GB',
        icon: 'memory',
        color: 'green',
        trend: 12.5,
      },
    });

    expect(screen.getByText('+12.5%')).toBeInTheDocument();
  });

  it('shows negative trend in red', () => {
    const { container } = render(StatsCard, {
      props: {
        title: 'Network',
        value: 1.2,
        unit: 'Mbps',
        icon: 'network',
        color: 'purple',
        trend: -5.3,
      },
    });

    const trendElement = screen.getByText('-5.3%');
    expect(trendElement).toBeInTheDocument();
    expect(trendElement.classList.contains('text-red-400')).toBe(true);
  });

  it('applies correct color classes', () => {
    const { container } = render(StatsCard, {
      props: {
        title: 'Test',
        value: 100,
        unit: 'units',
        icon: 'test',
        color: 'blue',
      },
    });

    const card = container.querySelector('.bg-blue-600\\/10');
    expect(card).toBeInTheDocument();
  });

  it('handles missing trend gracefully', () => {
    render(StatsCard, {
      props: {
        title: 'Storage',
        value: 256,
        unit: 'GB',
        icon: 'storage',
        color: 'orange',
      },
    });

    expect(screen.queryByText(/\+|-/)).not.toBeInTheDocument();
  });
});
